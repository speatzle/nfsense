package firewall

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetForwardRulesResult struct {
	ForwardRules []definitions.ForwardRule `json:"forward_rules"`
}

func (f *Firewall) GetForwardRules(ctx context.Context, params struct{}) (GetForwardRulesResult, error) {
	return GetForwardRulesResult{
		ForwardRules: f.ConfigManager.GetPendingConfig().Firewall.ForwardRules,
	}, nil
}

type CreateForwardRuleParameters struct {
	ForwardRule definitions.ForwardRule `json:"forward_rule"`
}

func (f *Firewall) CreateForwardRule(ctx context.Context, params CreateForwardRuleParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.ForwardRules = append(conf.Firewall.ForwardRules, params.ForwardRule)
	return struct{}{}, t.Commit()
}

type UpdateForwardRuleParameters struct {
	Index       uint64                  `json:"index"`
	ForwardRule definitions.ForwardRule `json:"forward_rule"`
}

func (f *Firewall) UpdateForwardRule(ctx context.Context, params UpdateForwardRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.ForwardRules) {
		return struct{}{}, fmt.Errorf("ForwardRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.ForwardRules[params.Index] = params.ForwardRule
	return struct{}{}, t.Commit()
}

type MoveForwardRuleParameters struct {
	Index   uint64 `json:"index"`
	ToIndex uint64 `json:"to_index"`
}

func (f *Firewall) MoveForwardRule(ctx context.Context, params MoveForwardRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.ForwardRules) {
		return struct{}{}, fmt.Errorf("ForwardRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	rule := conf.Firewall.ForwardRules[params.Index]
	sliceWithoutRule := append(conf.Firewall.ForwardRules[:params.Index], conf.Firewall.ForwardRules[params.Index+1:]...)
	newSlice := make([]definitions.ForwardRule, params.ToIndex+1)
	copy(newSlice, sliceWithoutRule[:params.ToIndex])
	newSlice[params.ToIndex] = rule
	conf.Firewall.ForwardRules = append(newSlice, sliceWithoutRule[params.ToIndex:]...)

	return struct{}{}, t.Commit()
}

type DeleteForwardRuleParameters struct {
	Index uint64 `json:"index"`
}

func (f *Firewall) DeleteForwardRule(ctx context.Context, params DeleteForwardRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.ForwardRules) {
		return struct{}{}, fmt.Errorf("ForwardRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.ForwardRules = append(conf.Firewall.ForwardRules[:params.Index], conf.Firewall.ForwardRules[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
