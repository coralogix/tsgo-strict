import type { Type1802 } from '../batch-09/file-1802.js';
export interface Type1803 {
  id: 1803;
  name: 'File1803';
  next: Type1802;
}

export function make1803(): Type1803 {
  return { id: 1803, name: 'File1803', next: null as unknown as Type1802 };
}
