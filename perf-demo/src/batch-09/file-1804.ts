import type { Type1803 } from '../batch-09/file-1803.js';
export interface Type1804 {
  id: 1804;
  name: 'File1804';
  next: Type1803;
}

export function make1804(): Type1804 {
  return { id: 1804, name: 'File1804', next: null as unknown as Type1803 };
}
