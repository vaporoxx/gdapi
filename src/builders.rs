use crate::{data::MapPack, error::Result, form, Client};
use std::{future::Future, pin::Pin};

type Request<'a, T> = Option<Pin<Box<dyn Future<Output = Result<T>> + 'a>>>;

pub struct MapPacksBuilder<'a> {
	client: &'a Client,
	page: Option<u8>,
	request: Request<'a, Vec<MapPack>>,
}

impl<'a> MapPacksBuilder<'a> {
	pub fn new(client: &'a Client) -> Self {
		Self {
			client,
			page: None,
			request: None,
		}
	}

	pub fn page(&mut self, page: u8) -> &mut Self {
		self.page = Some(page);
		self
	}
}

impl Future for MapPacksBuilder<'_> {
	type Output = Result<Vec<MapPack>>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
		if self.request.is_none() {
			let form = form::map_packs(self.page);
			let request = self.client.request("getGJMapPacks21", form);

			self.request = Some(Box::pin(request));
		}

		self.request.as_mut().unwrap().as_mut().poll(cx)
	}
}
