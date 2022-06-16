#![no_std]
use aidoku::{
	error::Result,
	prelude::*,
	std::{net::Request, String, Vec},
	Chapter, DeepLink, Filter, Manga, MangaContentRating, MangaPageResult, Page,
};
use lazy_static::lazy_static;
use mmrcms_template::{
	helper::text_with_newlines,
	template::{cache_manga_page, MMRCMSSource, CACHED_MANGA},
};

lazy_static! {
	static ref INSTANCE: MMRCMSSource = MMRCMSSource {
		base_url: "http://animaregia.net",
		lang: "pt-BR",
		category: "Categoria",
		..Default::default()
	};
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut result = INSTANCE
		.get_manga_list(filters, page)
		.unwrap_or(MangaPageResult {
			manga: Vec::new(),
			has_more: false,
		});
	result
		.manga
		.iter_mut()
		.for_each(|manga| manga.title = manga.title.replace(" (pt-br)", ""));

	Ok(result)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/{}/{}", INSTANCE.base_url, INSTANCE.manga_path, id);
	cache_manga_page(&url);
	let html = unsafe { CACHED_MANGA.clone().unwrap() };

	let title = html
		.select("h1.widget-title")
		.text()
		.read()
		.replace(" (pt-br)", "");
	let cover = html.select("img.img-thumbnail").attr("abs:src").read();
	let description = text_with_newlines(html.select("div.row div.well p"));
	let mut manga = Manga {
		id,
		title,
		cover,
		description,
		url,
		..Default::default()
	};

	for elem in html.select("li.list-group-item").array() {
		let node = elem.as_node();
		let text = node.text().read().to_lowercase();
		let end = text.find(':').unwrap_or(0);
		match &text.as_str()[..end] {
			"autor(es)" => {
				manga.author = node
					.select("a")
					.array()
					.map(|elem| elem.as_node().text().read())
					.collect::<Vec<_>>()
					.join(", ")
			}
			"artist(s)" => {
				manga.artist = node
					.select("a")
					.array()
					.map(|elem| elem.as_node().text().read())
					.collect::<Vec<_>>()
					.join(", ")
			}
			"categorias" => node
				.select("a")
				.array()
				.for_each(|elem| manga.categories.push(elem.as_node().text().read())),
			"status" => {
				manga.status = match node.select("span.label").text().read().trim() {
					"Completo" | "Concluído" => aidoku::MangaStatus::Completed,
					"Ativo" => aidoku::MangaStatus::Ongoing,
					_ => aidoku::MangaStatus::Unknown,
				}
			}
			_ => continue,
		}
	}
	(manga.nsfw, manga.viewer) = (INSTANCE.category_parser)(&html, manga.categories.clone());
	if html.select("div.alert.alert-danger").array().len() > 0 {
		manga.nsfw = MangaContentRating::Nsfw;
	}
	Ok(manga)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let mut result = INSTANCE.get_chapter_list(id).unwrap_or_default();

	result.iter_mut().for_each(|chapter| {
		let begin = chapter.title.find(" - ").unwrap_or(chapter.title.len() - 3) + 3;
		chapter.title = String::from(&chapter.title[begin..]);
	});
	Ok(result)
}

#[get_page_list]
fn get_page_list(id: String) -> Result<Vec<Page>> {
	INSTANCE.get_page_list(id)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	INSTANCE.modify_image_request(request)
}

#[handle_url]
fn handle_url(url: String) -> Result<DeepLink> {
	INSTANCE.handle_url(url)
}
