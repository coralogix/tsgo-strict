import type { Type1205 } from '../batch-06/file-1205.js';
export interface Type1206 {
  id: 1206;
  name: 'File1206';
  next: Type1205;
}

export function make1206(): Type1206 {
  return { id: 1206, name: 'File1206', next: null as unknown as Type1205 };
}
