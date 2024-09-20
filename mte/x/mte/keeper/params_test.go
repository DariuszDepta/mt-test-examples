package keeper_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	keepertest "mte/testutil/keeper"
	"mte/x/mte/types"
)

func TestGetParams(t *testing.T) {
	k, ctx := keepertest.MteKeeper(t)
	params := types.DefaultParams()

	require.NoError(t, k.SetParams(ctx, params))
	require.EqualValues(t, params, k.GetParams(ctx))
}
