# XML-AI Moved

I'm continuing the work on XML-AI over here [colbyn/AgenticWorkflow](https://github.com/colbyn/AgenticWorkflow).

# Work In Progress 

Working on a new DSL, a rewrite of the basic implementation from [WebCompiler](https://github.com/SuperSwiftDev/WebCompiler).

What I particularly love about the current system is that it can accommodate intermediate evaluations, for instance, chain of prompt reasoning via the breakpoint message where the LLM outputs will be substituted into a message element based on the provided role. E.g.
```html
<prompt name="question-1">
    <message role="system">You are a helpful assistant.</message>
    <message role="user">What is heavier, a kilo of feathers or a pound of steel?</message>
    <message-breakpoint role="assistant"></message-breakpoint>
    <message role="user">Explain your reasoning.</message>
    <message-breakpoint role="assistant"></message-breakpoint>
    <message role="user">Would you like to revise your answer?</message>
    <message-breakpoint role="assistant"></message-breakpoint>
    <message role="user">Finally, after considering your thoughts, please state just the answer.</message>
</prompt>
```

Overall, I'm attempting to generalize some techniques I've learned from this project as presented/demonstrated in this [YouTube video](https://youtu.be/nofJLw51xSk?si=587YwGXe4AB-2u3O): **'How I autogenerate massive (dictionary) datasets with ChatGPT/LLMs and why this matters'**.


## Background 

I've built complex pipelines that composed LLMs for generating 'publication quality' datasets. 

Here is an example, for generating dictionary datasets.

```liquid
<prompt name="process">
<message role="system">
{% if errors.size > 2 %}
You will fix all JSON syntax errors, JSON schema model validation errors, and then compile the corrected JSON dataset.
{% else %}
{{ system_message }}
{% endif %}
</message>
<message role="user">{{ instructions }}</message>
{% for error in errors %}
<message role="assistant">{{ error.response }}</message>
<message role="user">
{{ error.message }}
There is an issue with your compiled JSON object!
</message>
{% endfor %}
{% if errors.size > 0 %}
<message role="system">
Follow these instructions:
1. Take heed of the following schema:

{{json_schema}}

2. The task remains: populate the requested JSON object following the provided schema and then return the compiled data.

Hint: we do not want the schema but the data the schema represents.
</message>
{% endif %}
</prompt>
```

The goal here is to generalize the core ideas I've learned into a reusable, general purpose format. 


## Tentative DSL

```html
<schema src="./schema-1.json" id="schema-1"></schema>

<prompt name="fix-json-object" input:schema="of type Schema" input:json-value="of type String">
    <set response-format="json-object"></set>
    <msg role="system">
        <p>You previously returned invalid JSON that does not conform to the expected schema.</p>
        <p>Your task is to fix the syntax and ensure the structure matches the schema exactly.</p>
        <p>⚠️ Only return the **corrected JSON data** — do not include the schema, explanation, or any additional text.</p>
    </msg>
    <msg role="user">
        <p>Here is the malformed JSON:</p>
        <p from="json-value" format="pretty"></p>
    </msg>

    <msg role="user">
        <p>Here is the expected JSON schema:</p>
        <p from="schema" format="pretty"></p>
    </msg>
</prompt>



<prompt name="generate-user-profile" input:schema="of type Schema" input:json-value="of type String">
    <set response-format="json-object"></set>
    
    <msg role="system">
        <p>You are a strict JSON generator. Only return a JSON object conforming to the schema below.</p>
    </msg>

    <msg role="user">
        <p>Create a user profile for a new user named Sarah. Include her name, age, and whether she is a
        premium member.</p>
    </msg>

    <msg role="user">
        <p>Here is the JSON schema:</p>
        <p from="schema" format="pretty"></p>
    </msg>

    <breakpoint
        type="msg"
        role="assistant"
        verification="schema">
    </breakpoint>
</prompt>
```

Notes: 
- How should I accommodate validation loops? E.g.,
  > ```liquid
  > {% for error in errors %}
  > <message role="assistant">{{ error.response }}</message>
  > <message role="user">
  > {{ error.message }}
  > There is an issue with your compiled JSON object!
  > </message>
  > {% endfor %}
  > ```