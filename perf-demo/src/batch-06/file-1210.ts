import type { Type1209 } from '../batch-06/file-1209.js';
export interface Type1210 {
  id: 1210;
  name: 'File1210';
  next: Type1209;
}

export function make1210(): Type1210 {
  return { id: 1210, name: 'File1210', next: null as unknown as Type1209 };
}
