import { CreateConversationDto } from "./../modules/conversation.module";
import validateUUIDv4 from "./canister.util";

export default function sanitiseCreateConversationDto(
  model: CreateConversationDto
): asserts model is CreateConversationDto {
  if (!model.sourceId || !model.targetId) {
    throw new Error("Invalid CreateConversationDto payload");
  }

  validateUUIDv4(model.sourceId);
  validateUUIDv4(model.targetId);
}
