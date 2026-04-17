import type { Type1106 } from '../batch-05/file-1106.js';
export interface Type1107 {
  id: 1107;
  name: 'File1107';
  next: Type1106;
}

export function make1107(): Type1107 {
  return { id: 1107, name: 'File1107', next: null as unknown as Type1106 };
}
