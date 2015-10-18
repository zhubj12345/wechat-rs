use wechat::WeChatClient;
use wechat::client::WeChatGroup;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_group_create_update_and_delete() {
    let client = WeChatClient::new(APPID, SECRET);
    let group_api = WeChatGroup::new(&client);

    // create group
    let res = group_api.create("测试分组");
    assert!(res.is_ok());

    let group = res.unwrap();
    let group_id = group.id;

    // update group name
    let res = group_api.update(group_id, "Test Group");
    assert!(res.is_ok());

    let res = group_api.list();
    assert!(res.is_ok());
    let groups = res.unwrap();
    assert!(groups.len() > 0);

    // delete group
    let _ = group_api.delete(group_id);
}