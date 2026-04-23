import type { Type1138 } from '../batch-05/file-1138.js';
export interface Type1139 {
  id: 1139;
  name: 'File1139';
  next: Type1138;
}

export function make1139(): Type1139 {
  return { id: 1139, name: 'File1139', next: null as unknown as Type1138 };
}
