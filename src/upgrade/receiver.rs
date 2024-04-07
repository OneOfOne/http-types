use async_channel::TryRecvError;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::upgrade::Connection;

/// The receiving half of a channel to send an upgraded connection.
#[must_use = "Futures do nothing unless polled or .awaited"]
#[derive(Debug)]
pub struct Receiver {
	receiver: async_channel::Receiver<Connection>,
}

impl Receiver {
	/// Create a new instance of `Receiver`.
	#[allow(unused)]
	pub(crate) fn new(receiver: async_channel::Receiver<Connection>) -> Self {
		Self { receiver }
	}
}

impl Future for Receiver {
	type Output = Option<Connection>;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		match self.receiver.try_recv() {
			Ok(conn) => Poll::Ready(Some(conn)),
			Err(TryRecvError::Closed) => Poll::Ready(None),
			Err(TryRecvError::Empty) => Poll::Pending,
		}
	}
}
