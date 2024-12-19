export function initVertices(aspectRatio: number): number[][] {
  const positions = new Array<number[]>();
  positions.push([-aspectRatio, -1]);
  positions.push([+aspectRatio, -1]);
  positions.push([-aspectRatio, +1]);
  positions.push([+aspectRatio, +1]);
  return positions;
}
