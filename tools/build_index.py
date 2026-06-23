import json, glob, os, re
items=[]; tot={"nodes":0,"edges":0,"conflict_axes":0,"edge_cases":0,"workflow":0,"competency_questions":0,"est_tokens":0}
for kb in sorted(glob.glob("knowledge_base/knowledge_searcher/*.kb.json")):
    base=kb[:-len(".kb.json")]; unit=os.path.basename(base)
    m=json.load(open(base+".metrics.json"))
    v=json.load(open(base+".validation.json"))
    c=m["counts"]
    dom,_,sub=unit.partition("__")
    it={"unit":unit,"domain_code":dom,"subdomain":sub,"kb_path":kb,
        "validator":v["overall_status"],"est_tokens":m["est_tokens"],
        "nodes":c.get("nodes"),"edges":c.get("edges"),
        "conflict_axes":c.get("conflict_axes"),"edge_cases":c.get("edge_cases"),
        "workflow":c.get("workflow"),"competency_questions":c.get("competency_questions")}
    items.append(it)
    for k in tot:
        key="est_tokens" if k=="est_tokens" else k
        val=m["est_tokens"] if k=="est_tokens" else c.get(k,0)
        tot[k]+=val or 0
index={
 "_directive":"losslessly compressed, token-efficient, information-dense, fully detailed, machine-facing; optimized for model parsing over human readability",
 "set":"knowledge_searcher",
 "purpose":"foundational KB set: where/how to find, retrieve, evaluate, and synthesize authoritative knowledge; consulted by every Phase B idea->specialist run",
 "grain":"subdomain (dense)","engine":"schema/kb_generator_v1.4.1.txt",
 "gate":"validators/kb_validator.py --mode dense (35 checks)",
 "kb_count":len(items),"all_pass":all(i["validator"]=="pass" for i in items),
 "totals":tot,
 "domains":sorted(set(i["domain_code"] for i in items)),
 "kbs":items
}
open("knowledge_base/knowledge_searcher/INDEX.json","w").write(json.dumps(index,indent=2))
print("KBs:",len(items),"| all_pass:",index["all_pass"])
print("TOTAL nodes:",tot["nodes"],"edges:",tot["edges"],"conflict_axes:",tot["conflict_axes"],
      "edge_cases:",tot["edge_cases"],"workflow:",tot["workflow"],"CQs:",tot["competency_questions"])
print("TOTAL est_tokens:",tot["est_tokens"],"(~",round(tot["est_tokens"]/1000),"k)")
print("domains:",index["domains"])
