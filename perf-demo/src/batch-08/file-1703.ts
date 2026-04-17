import type { Type1702 } from '../batch-08/file-1702.js';
export interface Type1703 {
  id: 1703;
  name: 'File1703';
  next: Type1702;
}

export function make1703(): Type1703 {
  return { id: 1703, name: 'File1703', next: null as unknown as Type1702 };
}
