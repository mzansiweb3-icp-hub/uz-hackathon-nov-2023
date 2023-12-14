import { CreateConversationDto } from "./../modules/conversation.module";
import validateUUID from "./canister.util";

export default function sanitiseCreateConversationDto(
  model: CreateConversationDto
): asserts model is CreateConversationDto {
  if (!model.sourceId || !model.targetId) {
    throw new Error("Invalid CreateConversationDto payload");
  }

  validateUUID(model.sourceId, model.targetId);
}
