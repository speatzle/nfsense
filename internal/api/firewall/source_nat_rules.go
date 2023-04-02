package firewall

import (
	"context"
	"fmt"

	"nfsense.net/nfsense/internal/definitions"
)

type GetSourceNATRulesResult struct {
	SourceNATRules []definitions.SourceNATRule `json:"source_nat_rules"`
}

func (f *Firewall) GetSourceNATRules(ctx context.Context, params struct{}) (GetSourceNATRulesResult, error) {
	return GetSourceNATRulesResult{
		SourceNATRules: f.ConfigManager.GetPendingConfig().Firewall.SourceNATRules,
	}, nil
}

type CreateSourceNATRuleParameters struct {
	SourceNATRule definitions.SourceNATRule `json:"source_nat_rule"`
}

func (f *Firewall) CreateSourceNATRule(ctx context.Context, params CreateSourceNATRuleParameters) (struct{}, error) {
	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.SourceNATRules = append(conf.Firewall.SourceNATRules, params.SourceNATRule)
	return struct{}{}, t.Commit()
}

type UpdateSourceNATRuleParameters struct {
	Index         uint64                    `json:"index"`
	SourceNATRule definitions.SourceNATRule `json:"source_nat_rule"`
}

func (f *Firewall) UpdateSourceNATRule(ctx context.Context, params UpdateSourceNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.SourceNATRules) {
		return struct{}{}, fmt.Errorf("SourceNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.SourceNATRules[params.Index] = params.SourceNATRule
	return struct{}{}, t.Commit()
}

type MoveSourceNATRuleParameters struct {
	Index   uint64 `json:"index"`
	ToIndex uint64 `json:"to_index"`
}

func (f *Firewall) MoveSourceNATRule(ctx context.Context, params DeleteSourceNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.SourceNATRules) {
		return struct{}{}, fmt.Errorf("SourceNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.SourceNATRules = append(conf.Firewall.SourceNATRules[:params.Index], conf.Firewall.SourceNATRules[params.Index+1:]...)
	return struct{}{}, t.Commit()
}

type DeleteSourceNATRuleParameters struct {
	Index uint64 `json:"index"`
}

func (f *Firewall) DeleteSourceNATRule(ctx context.Context, params DeleteSourceNATRuleParameters) (struct{}, error) {
	if int(params.Index) >= len(f.ConfigManager.GetPendingConfig().Firewall.SourceNATRules) {
		return struct{}{}, fmt.Errorf("SourceNATRule does not Exist")
	}

	t, conf := f.ConfigManager.StartTransaction()
	defer t.Discard()

	conf.Firewall.SourceNATRules = append(conf.Firewall.SourceNATRules[:params.Index], conf.Firewall.SourceNATRules[params.Index+1:]...)
	return struct{}{}, t.Commit()
}
