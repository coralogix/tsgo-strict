import type { Type3010 } from '../batch-15/file-3010.js';
export interface Type3011 {
  id: 3011;
  name: 'File3011';
  next: Type3010;
}

export function make3011(): Type3011 {
  return { id: 3011, name: 'File3011', next: null as unknown as Type3010 };
}
