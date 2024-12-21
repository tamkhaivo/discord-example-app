import { getRandomEmoji } from "../utils.js";
import { InteractionResponseType } from "discord-interactions";

export async function test(req, res) {
    const emoji = getRandomEmoji();
    return res.send({
        type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
        data: {
        content: `Hello, ${emoji}`,
        },
    });
}x


