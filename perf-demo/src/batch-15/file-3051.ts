import type { Type3050 } from '../batch-15/file-3050.js';
export interface Type3051 {
  id: 3051;
  name: 'File3051';
  next: Type3050;
}

export function make3051(): Type3051 {
  return { id: 3051, name: 'File3051', next: null as unknown as Type3050 };
}
