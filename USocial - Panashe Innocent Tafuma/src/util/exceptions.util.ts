export default function getErrorMessage(error: any): string {
  return error?.message ?? error ?? "Unkown Error";
}
