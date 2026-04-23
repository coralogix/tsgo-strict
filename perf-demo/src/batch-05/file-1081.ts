import type { Type1080 } from '../batch-05/file-1080.js';
export interface Type1081 {
  id: 1081;
  name: 'File1081';
  next: Type1080;
}

export function make1081(): Type1081 {
  return { id: 1081, name: 'File1081', next: null as unknown as Type1080 };
}
