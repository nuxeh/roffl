#[derive(Serialize, Deserialize)]
struct Channel<'a> {
    name: &'a str,
};

#[derive(Serialize, Deserialize)]
struct ChannelBlock {
    channels: Vec<Channel>,
};

#[derive(Serialize, Deserialize)]
struct ChannelListing {
    channels: Vec<String>,
};
