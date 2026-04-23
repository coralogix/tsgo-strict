import type { Type1207 } from '../batch-06/file-1207.js';
export interface Type1208 {
  id: 1208;
  name: 'File1208';
  next: Type1207;
}

export function make1208(): Type1208 {
  return { id: 1208, name: 'File1208', next: null as unknown as Type1207 };
}
