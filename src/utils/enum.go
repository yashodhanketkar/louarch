package utils

import (
	"fmt"
	"strings"
)

type StringEnum[T ~string] struct {
	values map[T]func()
}

func NewEnum[T ~string](m map[T]func()) *StringEnum[T] {
	return &StringEnum[T]{values: m}
}

func (e *StringEnum[T]) Call(v T) error {
	if fn, ok := e.values[v]; ok {
		fn()
		return nil
	}

	return fmt.Errorf("Invalid value %q\nAvailable values are: %s", v, e.Available())
}

func (e *StringEnum[T]) Values() []string {
	keys := make([]string, 0, len(e.values))
	for k := range e.values {
		keys = append(keys, string(k))
	}

	return keys
}

func (e *StringEnum[T]) Available() string {
	keys := e.Values()
	return strings.Join(keys, ", ")
}
