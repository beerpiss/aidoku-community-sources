#![no_std]
use aidoku::{
	error::Result, prelude::*, std::String, std::Vec, Chapter, DeepLink, Filter, Listing, Manga,
	MangaPageResult, MangaStatus, Page,
};

use madara_template::template;

fn get_data() -> template::MadaraSiteData {
	let data: template::MadaraSiteData = template::MadaraSiteData {
		base_url: String::from("https://reaperscansar.com"),
		source_path: String::from("series"),
		alt_ajax: true,
		status_filter_ongoing: String::from("مستمر"),
		status_filter_completed: String::from("مكتمل"),
		status_filter_cancelled: String::from("ملغى"),
		status_filter_on_hold: String::from("متوقفة"),
		status: |html| {
			let status_str = html
				.select("div.post-content_item:contains(الحالة) div.summary-content")
				.text()
				.read()
				.to_lowercase();
			match status_str.as_str() {
				"مستمر" => MangaStatus::Ongoing,
				"مكتمل" => MangaStatus::Completed,
				"ملغى" => MangaStatus::Cancelled,
				"On Hold" => MangaStatus::Hiatus,
				_ => MangaStatus::Unknown,
			}
		},
		..Default::default()
	};
	data
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	template::get_manga_list(filters, page, get_data())
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	template::get_manga_listing(get_data(), listing, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	template::get_manga_details(id, get_data())
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	template::get_chapter_list(id, get_data())
}

#[get_page_list]
fn get_page_list(chapter_id: String, _manga_id: String) -> Result<Vec<Page>> {
	template::get_page_list(chapter_id, get_data())
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	template::handle_url(url, get_data())
}
