package firewall

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions/firewall"
)

type GetDestinationNATRulesResult struct {
	DestinationNATRules []firewall.DestinationNATRule `json:"destination_nat_rules"`
}

func (f *Firewall) GetDestinationNATRules(ctx context.Context, params struct{}) (GetDestinationNATRulesResult, error) {
	return GetDestinationNATRulesResult{
		DestinationNATRules: f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules,
	}, nil
}

type CreateDestinationNATRuleParameters struct {
	DestinationNATRule firewall.DestinationNATRule `json:"destination_nat_rule"`
}

func (f *Firewall) CreateDestinationNATRule(ctx context.Context, params CreateDestinationNATRuleParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.DestinationNATRules = append(conf.Firewall.DestinationNATRules, params.DestinationNATRule)
	return struct{}{}, t.Commit()
}

type UpdateDestinationNATRuleParameters struct {
	Index              uint64                      `json:"index"`
	DestinationNATRule firewall.DestinationNATRule `json:"destination_nat_rule"`
}

func (f *Firewall) UpdateDestinationNATRule(ctx context.Context, params UpdateDestinationNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules) {
		return struct{}{}, fmt.Errorf("DestinationNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.DestinationNATRules[params.Index] = params.DestinationNATRule
	return struct{}{}, t.Commit()
}

type MoveDestinationNATRuleParameters struct {
	Index   uint64 `json:"index"`
	ToIndex uint64 `json:"to_index"`
}

func (f *Firewall) MoveDestinationNATRule(ctx context.Context, params MoveDestinationNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules) {
		return struct{}{}, fmt.Errorf("DestinationNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	rule := conf.Firewall.DestinationNATRules[params.Index]
	sliceWithoutRule := append(conf.Firewall.DestinationNATRules[:params.Index], conf.Firewall.DestinationNATRules[params.Index+1:]...)
	newSlice := make([]firewall.DestinationNATRule, params.ToIndex+1)
	copy(newSlice, sliceWithoutRule[:params.ToIndex])
	newSlice[params.ToIndex] = rule
	conf.Firewall.DestinationNATRules = append(newSlice, sliceWithoutRule[params.ToIndex:]...)

	return struct{}{}, t.Commit()
}

type DeleteDestinationNATRuleParameters struct {
	Index uint64 `json:"index"`
}

func (f *Firewall) DeleteDestinationNATRule(ctx context.Context, params DeleteDestinationNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.DestinationNATRules) {
		return struct{}{}, fmt.Errorf("DestinationNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.DestinationNATRules = append(conf.Firewall.DestinationNATRules[:params.Index], conf.Firewall.DestinationNATRules[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
