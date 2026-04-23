import type { Type3009 } from '../batch-15/file-3009.js';
export interface Type3010 {
  id: 3010;
  name: 'File3010';
  next: Type3009;
}

export function make3010(): Type3010 {
  return { id: 3010, name: 'File3010', next: null as unknown as Type3009 };
}
