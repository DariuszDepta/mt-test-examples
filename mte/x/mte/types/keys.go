package types

const (
	// ModuleName defines the module name
	ModuleName = "mte"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_mte"
)

var (
	ParamsKey = []byte("p_mte")
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}
