import type { Type42 } from '../batch-00/file-0042.js';
export interface Type43 {
  id: 43;
  name: 'File43';
  next: Type42;
}

export function make43(): Type43 {
  return { id: 43, name: 'File43', next: null as unknown as Type42 };
}
