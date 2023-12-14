export class BadRequestException extends Error {
  public constructor(message: string = "Bad Request") {
    super(message);

    Object.setPrototypeOf(this, BadRequestException.prototype);
    this.name = "BadRequestException";
  }
}
