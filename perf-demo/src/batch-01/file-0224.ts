import type { Type223 } from '../batch-01/file-0223.js';
export interface Type224 {
  id: 224;
  name: 'File224';
  next: Type223;
}

export function make224(): Type224 {
  return { id: 224, name: 'File224', next: null as unknown as Type223 };
}
