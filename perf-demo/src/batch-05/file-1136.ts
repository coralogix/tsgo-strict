import type { Type1135 } from '../batch-05/file-1135.js';
export interface Type1136 {
  id: 1136;
  name: 'File1136';
  next: Type1135;
}

export function make1136(): Type1136 {
  return { id: 1136, name: 'File1136', next: null as unknown as Type1135 };
}
