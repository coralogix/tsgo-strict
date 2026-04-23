import type { Type365 } from '../batch-01/file-0365.js';
export interface Type366 {
  id: 366;
  name: 'File366';
  next: Type365;
}

export function make366(): Type366 {
  return { id: 366, name: 'File366', next: null as unknown as Type365 };
}
