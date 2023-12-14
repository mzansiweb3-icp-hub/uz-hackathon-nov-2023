abstract class CanisterUtil {
  public static readonly UUID_REGEXP =
    /^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$/i;
}

export default function validateUUID(...arg: Array<unknown>): void {
  function validate(arg: string): boolean {
    return CanisterUtil.UUID_REGEXP.test(arg);
  }

  arg.forEach((uuid) => {
    if (!uuid || typeof uuid !== "string") {
      throw new Error(`Invalid UUID: ${uuid}`);
    }

    if (!validate(uuid)) {
      throw new Error(`UUID Assertion failed for ${uuid}`);
    }
  });
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
