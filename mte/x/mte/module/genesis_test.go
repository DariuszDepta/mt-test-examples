package mte_test

import (
	"testing"

	keepertest "mte/testutil/keeper"
	"mte/testutil/nullify"
	mte "mte/x/mte/module"
	"mte/x/mte/types"

	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.MteKeeper(t)
	mte.InitGenesis(ctx, k, genesisState)
	got := mte.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	// this line is used by starport scaffolding # genesis/test/assert
}
