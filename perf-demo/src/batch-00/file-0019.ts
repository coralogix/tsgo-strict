import type { Type18 } from '../batch-00/file-0018.js';
export interface Type19 {
  id: 19;
  name: 'File19';
  next: Type18;
}

export function make19(): Type19 {
  return { id: 19, name: 'File19', next: null as unknown as Type18 };
}
