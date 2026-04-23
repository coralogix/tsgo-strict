import type { Type1312 } from '../batch-06/file-1312.js';
export interface Type1313 {
  id: 1313;
  name: 'File1313';
  next: Type1312;
}

export function make1313(): Type1313 {
  return { id: 1313, name: 'File1313', next: null as unknown as Type1312 };
}
