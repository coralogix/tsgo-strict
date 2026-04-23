import type { Type3040 } from '../batch-15/file-3040.js';
export interface Type3041 {
  id: 3041;
  name: 'File3041';
  next: Type3040;
}

export function make3041(): Type3041 {
  return { id: 3041, name: 'File3041', next: null as unknown as Type3040 };
}
