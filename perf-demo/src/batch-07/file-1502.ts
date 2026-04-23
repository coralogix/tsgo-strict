import type { Type1501 } from '../batch-07/file-1501.js';
export interface Type1502 {
  id: 1502;
  name: 'File1502';
  next: Type1501;
}

export function make1502(): Type1502 {
  return { id: 1502, name: 'File1502', next: null as unknown as Type1501 };
}
