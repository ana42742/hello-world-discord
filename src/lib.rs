use discord_flows::{model::ChannelId, model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let discord_token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(discord_token);
    bot.listen(|msg| handler(&bot, msg)).await;
    Ok(())
}

async fn handler(bot: &ProvidedBot, msg: Message) {
    logger::init();
    let discord = bot.get_client();

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let channel_id = msg.channel_id;
    let user_message = msg.content.to_lowercase();
    let mut response = "I'm sorry, I didn't understand your question.";

    if user_message.contains("hello") {
        response = "Hello there! How can I help you today?";
    } else if user_message.contains("explain image segmentation") {
        response = "Image segmentation is a computer vision task that divides an image into meaningful regions,
        grouping pixels with similar characteristics. It plays a vital role in object recognition and scene analysis.
        Popular methods include region-based, edge-based, and deep learning-based approaches, such as U-Net and Mask R-CNN.
        (Reference: R. C. Gonzalez and R. E. Woods, Digital Image Processing, 3rd Edition, 2007.)";
    } else if user_message.contains("ffmpeg") {
        response = "FFmpeg is a powerful open-source multimedia framework that facilitates audio and video processing,
        encoding, decoding, transcoding, and streaming. It supports a wide range of formats and codecs, making it a versatile
        tool for media manipulation. (Reference: FFmpeg official website - https://ffmpeg.org/about.html)";
    } else if user_message.contains("WasmEdge") {
        response = "WasmEdge is an open-source, high-performance runtime for WebAssembly (Wasm) designed for edge
        computing and the Internet of Things (IoT). It executes Wasm modules securely and efficiently, enabling seamless
        integration of multiple programming languages. (Reference: WasmEdge GitHub repository - https://github.com/WasmEdge/WasmEdge)";
    } else if user_message.contains("object detection") {
        response = "An object detection image processing pipeline involves several steps. First, preprocess the image
        by resizing and normalizing it. Then, extract relevant features through techniques like edge detection. Next, propose
        potential object regions using methods such as selective search. Extract features from these regions and classify them
        using a model like a Convolutional Neural Network (CNN). Apply non-maximum suppression to eliminate overlapping detections.
        Refine the results through post-processing, filtering out false positives. Finally, visualize the detected objects by
        drawing bounding boxes. Modern methods integrate these steps into deep learning networks for more accurate and efficient
        object detection, such as YOLO and Faster R-CNN.";
    } else if user_message.contains("bye") {
        // Respond to "bye" message
        response = "Good-bye!";
    }

    // let resp = format!("Welcome to flows.network.\nYou just said: '{}'.\nLearn more at: https://github.com/flows-network/hello-world\n", msg.content);

    _ = discord
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": response
            }),
        )
        .await;
}
