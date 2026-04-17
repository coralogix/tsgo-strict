import type { Type1206 } from '../batch-06/file-1206.js';
export interface Type1207 {
  id: 1207;
  name: 'File1207';
  next: Type1206;
}

export function make1207(): Type1207 {
  return { id: 1207, name: 'File1207', next: null as unknown as Type1206 };
}
