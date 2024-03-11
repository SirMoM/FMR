use chrono::{Datelike, Utc};

use fakes::FakeMessageGetter;

use crate::discord_api::messages::{GetMsgs, MessageService};
use crate::sort_messages_by_upvote;

mod fakes;

#[test]
fn test_get_memes_for_month() {
    let struct_to_test = MessageService::new(FakeMessageGetter {});
    let month = Utc::now().with_day(1).expect("???"); // TODO: static date

    let result = struct_to_test.get_memes_for_month(&month);

    assert_eq!(result.len(), 5)
}
#[test]
fn test_sort_messages_by_upvote() {
    let struct_to_test = MessageService::new(FakeMessageGetter {});
    let month = Utc::now().with_day(1).expect("???"); // TODO: static date

    let all_msgs = struct_to_test.client.get_messages(None, None).expect("???");

    let sorted_msgs = sort_messages_by_upvote(all_msgs.iter().collect());

    assert_eq!(sorted_msgs.len(), 2);
    assert_eq!(sorted_msgs[0].id, "1213768968272085042");
    assert_eq!(sorted_msgs[1].id, "1215354063861055520");
}
