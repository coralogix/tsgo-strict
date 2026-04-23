import type { Type665 } from '../batch-03/file-0665.js';
export interface Type666 {
  id: 666;
  name: 'File666';
  next: Type665;
}

export function make666(): Type666 {
  return { id: 666, name: 'File666', next: null as unknown as Type665 };
}
