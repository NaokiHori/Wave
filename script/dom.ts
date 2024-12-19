export function getCanvasElement(id: string): HTMLCanvasElement {
  const canvas: HTMLElement | null = document.getElementById(id);
  if (null === canvas) {
    throw new Error(`failed to find a canvas element: ${id}`);
  }
  return canvas as HTMLCanvasElement;
}
