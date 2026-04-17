import type { Type2501 } from '../batch-12/file-2501.js';
export interface Type2502 {
  id: 2502;
  name: 'File2502';
  next: Type2501;
}

export function make2502(): Type2502 {
  return { id: 2502, name: 'File2502', next: null as unknown as Type2501 };
}
