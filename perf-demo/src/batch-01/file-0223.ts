import type { Type222 } from '../batch-01/file-0222.js';
export interface Type223 {
  id: 223;
  name: 'File223';
  next: Type222;
}

export function make223(): Type223 {
  return { id: 223, name: 'File223', next: null as unknown as Type222 };
}
