import type { Type3011 } from '../batch-15/file-3011.js';
export interface Type3012 {
  id: 3012;
  name: 'File3012';
  next: Type3011;
}

export function make3012(): Type3012 {
  return { id: 3012, name: 'File3012', next: null as unknown as Type3011 };
}
