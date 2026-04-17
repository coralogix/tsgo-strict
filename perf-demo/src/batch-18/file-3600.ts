import type { Type3599 } from '../batch-17/file-3599.js';
export interface Type3600 {
  id: 3600;
  name: 'File3600';
  next: Type3599;
}

export function make3600(): Type3600 {
  return { id: 3600, name: 'File3600', next: null as unknown as Type3599 };
}
