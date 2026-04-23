import type { Type232 } from '../batch-01/file-0232.js';
export interface Type233 {
  id: 233;
  name: 'File233';
  next: Type232;
}

export function make233(): Type233 {
  return { id: 233, name: 'File233', next: null as unknown as Type232 };
}
