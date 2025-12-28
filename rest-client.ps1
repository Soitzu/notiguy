
$Params = @{
    Uri = "http://localhost:3000/tickets"
    Method = "Get"
}

Invoke-RestMethod @Params



$Params = @{
    Uri = "http://localhost:3000/tickets"
    Method = "Post"
    Body = @{
        title = "Titel 1"
        content = "blablabla"
    } | ConvertTo-Json
    ContentType = "application/json"

}

Invoke-RestMethod @Params