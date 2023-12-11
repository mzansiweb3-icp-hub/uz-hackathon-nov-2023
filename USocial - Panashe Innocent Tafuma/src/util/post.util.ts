import { PartialPost, Post, UpdatePostDto } from "../modules";

export abstract class PostConstants {
  public static readonly MAX_CAPTION_LENGTH = 400 as const;
  public static readonly MIN_CAPTION_LENGTH = 1 as const;
}

export function validateCaption(caption: string): asserts caption is string {
  if (typeof caption !== "string") {
    throw new Error(`Invalid caption: ${caption}`);
  }

  if (
    caption.length > PostConstants.MAX_CAPTION_LENGTH ||
    caption.length < PostConstants.MIN_CAPTION_LENGTH
  ) {
    throw new Error(
      `caption must be between ${PostConstants.MIN_CAPTION_LENGTH} and ${PostConstants.MAX_CAPTION_LENGTH} characters long`
    );
  }
}

export function validateTags(
  tags: Array<string>
): asserts tags is Array<string> {
  if (!tags || !Array.isArray(tags)) {
    throw new Error(`Invalid tags`);
  }

  tags.forEach((tag) => {
    if (!tag.startsWith("#")) {
      throw new Error(`Invalid tag: ${tag}`);
    }
  });
}

export function sanitisePostUpdate(
  post: Post,
  update: UpdatePostDto
): PartialPost {
  let caption = post.caption;
  if (update?.caption && typeof update.caption === "string") {
    validateCaption(update.caption);
    caption = update.caption;
  }

  let tags = post.tags;
  if (update?.tags && Array.isArray(update.tags)) {
    validateTags(update.tags);
    tags = update.tags;
  }

  return {
    caption,
    tags,
  };
}
