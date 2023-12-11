abstract class CanisterUtil {
  public static readonly UUID4_REGEXP =
    /^[0-9A-F]{8}-[0-9A-F]{4}-[4][0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}$/i;
}

export default function validateUUIDv4(uuid: unknown): asserts uuid is string {
  if (!uuid || typeof uuid !== "string") {
    throw new Error("Invalid UUID");
  }

  if (!CanisterUtil.UUID4_REGEXP.test(uuid)) {
    throw new Error("UUID Assertion failed");
  }
}

export function validateBoolean(
  arg: unknown,
  value?: string
): asserts arg is boolean {
  if (!arg || typeof arg !== "boolean") {
    throw new Error(`${value ?? "parameter"} must be a boolean`);
  }
}

export function validateUnixDate(arg: unknown): void {
  if (!arg || typeof arg !== "number" || typeof arg !== "bigint") {
    throw new Error(`Invalid unix time: ${arg}`);
  }

  if (isNaN(new Date(arg).getTime())) {
    throw new Error(`Unix time: ${arg} failed date assertion test`);
  }
}
