export class NotFoundException extends Error {
  constructor(message: string | undefined) {
    super(message);

    Object.setPrototypeOf(this, NotFoundException.prototype);
    this.name = "NotFoundError";
  }
}
